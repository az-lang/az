macro_rules! define_baseless_py_dataclass {
    // positional-only fields variant
    (
        $name:ident, $py_module_name:literal, $py_name:literal,
        $($field_name:ident: $field_type:ty,)+
    ) => {
        #[derive(Clone, PartialEq)]
        #[pyo3::pyclass(
            module = $py_module_name, name = $py_name, frozen, get_all
        )]
        pub(crate) struct $name {
            $($field_name: $field_type,)+
        }

        #[pyo3::pymethods]
        impl $name {
            #[new]
            #[pyo3(signature = ($($field_name,)+ /))]
            fn new(
                $($field_name: $field_type,)+
            ) -> Self {
                Self {
                    $($field_name,)+
                }
            }

            fn __repr__(&self, py: pyo3::Python<'_>) -> pyo3::PyResult<String> {
                crate::traits::Repr::repr(self, py)
            }

            fn __richcmp__(
                &self,
                other: &pyo3::Bound<'_, pyo3::PyAny>,
                op: pyo3::pyclass::CompareOp,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                crate::traits::RichCmp::rich_cmp(self, other, op, py)
            }
        }

        impl crate::traits::Repr for $name {
            fn repr(&self, py: pyo3::Python<'_>) -> pyo3::PyResult<String> {
                Ok(
                    format!(
                        "{}({})", <Self as pyo3::PyTypeInfo>::NAME,
                        [
                            $(self.$field_name.repr(py)?,)+
                        ].join(", ")
                    )
                )
            }
        }
    };
    // mixed fields variant
    (
        $name:ident, $py_module_name:literal, $py_name:literal,
        $($positional_field_name:ident: $positional_field_type:ty,)* *,
        $($keyword_field_name:ident: $keyword_field_type:ty,)+
    ) => {
        #[derive(Clone, PartialEq)]
        #[pyo3::pyclass(
            module = $py_module_name, name = $py_name, frozen, get_all
        )]
        pub(crate) struct $name {
            $($positional_field_name: $positional_field_type,)*
            $($keyword_field_name: $keyword_field_type,)+
        }

        #[pyo3::pymethods]
        impl $name {
            #[new]
            #[pyo3(
                signature = ($($positional_field_name,)* /,
                *, $($keyword_field_name,)+)
            )]
            fn new(
                $($positional_field_name: $positional_field_type,)*
                $($keyword_field_name: $keyword_field_type,)+
            ) -> Self {
                Self {
                    $($positional_field_name,)*
                    $($keyword_field_name,)+
                }
            }

            fn __repr__(&self, py: pyo3::Python<'_>) -> pyo3::PyResult<String> {
                crate::traits::Repr::repr(self, py)
            }

            fn __richcmp__(
                &self,
                other: &pyo3::Bound<'_, pyo3::PyAny>,
                op: pyo3::pyclass::CompareOp,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                crate::traits::RichCmp::rich_cmp(self, other, op, py)
            }
        }

        impl crate::traits::Repr for $name {
            fn repr(&self, py: pyo3::Python<'_>) -> pyo3::PyResult<String> {
                Ok(
                    format!(
                        "{}({})", <Self as pyo3::PyTypeInfo>::NAME,
                        [
                            $(
                                crate::traits::Repr::repr(
                                    &self.$positional_field_name, py
                                )?,
                            )*
                            $(
                                format!(
                                    "{}={}",
                                    stringify!($keyword_field_name),
                                    crate::traits::Repr::repr(
                                        &self.$keyword_field_name, py
                                    )?
                                ),
                            )+
                        ].join(", ")
                    )
                )
            }
        }
    };
}

macro_rules! impl_ordered_rich_cmp_for_baseless_py_class {
    ($baseless:ty) => {
        impl crate::traits::RichCmp for $baseless {
            fn rich_cmp(
                &self,
                other: &pyo3::Bound<'_, pyo3::PyAny>,
                op: pyo3::pyclass::CompareOp,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                if let Ok(other) = pyo3::types::PyAnyMethods::extract::<
                    pyo3::PyRef<'_, $baseless>,
                >(other)
                {
                    Ok(match op {
                        pyo3::pyclass::CompareOp::Eq => {
                            pyo3::IntoPy::into_py(self.eq(&other), py)
                        }
                        pyo3::pyclass::CompareOp::Ge => {
                            pyo3::IntoPy::into_py(self.ge(&other), py)
                        }
                        pyo3::pyclass::CompareOp::Gt => {
                            pyo3::IntoPy::into_py(self.gt(&other), py)
                        }
                        pyo3::pyclass::CompareOp::Le => {
                            pyo3::IntoPy::into_py(self.le(&other), py)
                        }
                        pyo3::pyclass::CompareOp::Lt => {
                            pyo3::IntoPy::into_py(self.lt(&other), py)
                        }
                        pyo3::pyclass::CompareOp::Ne => {
                            pyo3::IntoPy::into_py(self.ne(&other), py)
                        }
                    })
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    };
}

macro_rules! impl_unordered_rich_cmp_for_baseless_py_class {
    ($baseless:ty) => {
        impl crate::traits::RichCmp for $baseless {
            fn rich_cmp(
                &self,
                other: &pyo3::Bound<'_, pyo3::PyAny>,
                op: pyo3::pyclass::CompareOp,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                if let Ok(other) = pyo3::types::PyAnyMethods::extract::<
                    pyo3::PyRef<'_, $baseless>,
                >(other)
                {
                    Ok(match op {
                        pyo3::pyclass::CompareOp::Eq => {
                            pyo3::IntoPy::into_py(self.eq(&other), py)
                        }
                        pyo3::pyclass::CompareOp::Ne => {
                            pyo3::IntoPy::into_py(self.ne(&other), py)
                        }
                        _ => py.NotImplemented(),
                    })
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    };
}

macro_rules! impl_unordered_rich_cmp_for_derived_py_class {
    ($derived:ty, $base:ty) => {
        impl crate::traits::RichCmp for $derived {
            fn rich_cmp(
                &self,
                other: &pyo3::Bound<'_, pyo3::PyAny>,
                op: pyo3::pyclass::CompareOp,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                if pyo3::types::PyAnyMethods::extract::<pyo3::PyRef<'_, $base>>(other).is_ok() {
                    if let Ok(other) =
                        pyo3::types::PyAnyMethods::extract::<pyo3::PyRef<'_, $derived>>(other)
                    {
                        Ok(match op {
                            pyo3::pyclass::CompareOp::Eq => {
                                pyo3::IntoPy::into_py(self.eq(&other), py)
                            }
                            pyo3::pyclass::CompareOp::Ne => {
                                pyo3::IntoPy::into_py(self.ne(&other), py)
                            }
                            _ => py.NotImplemented(),
                        })
                    } else {
                        Ok(pyo3::IntoPy::into_py(false, py))
                    }
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    };
}

pub(super) use define_baseless_py_dataclass;
pub(super) use impl_ordered_rich_cmp_for_baseless_py_class;
pub(super) use impl_unordered_rich_cmp_for_baseless_py_class;
pub(super) use impl_unordered_rich_cmp_for_derived_py_class;
