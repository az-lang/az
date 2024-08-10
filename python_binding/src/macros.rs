macro_rules! define_baseless_py_dataclass {
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

            fn __getnewargs_ex__<'py>(
                &'py self,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<(
                pyo3::Bound<'py, pyo3::types::PyTuple>,
                pyo3::Bound<'py, pyo3::types::PyDict>
            )> {
                let kwargs = pyo3::types::PyDict::new_bound(py);
                $(
                pyo3::types::PyDictMethods::set_item(
                    &kwargs,
                    stringify!($keyword_field_name),
                    pyo3::IntoPy::into_py(self.$keyword_field_name.clone(), py),
                )?;
                )+
                let args: Vec<pyo3::PyObject> = vec![
                    $(
                        pyo3::IntoPy::into_py(&self.$positional_field_name, py),
                    )*
                ];
                Ok((pyo3::types::PyTuple::new_bound(py, &args), kwargs))
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
                        "{}({})", <Self as pyo3::PyTypeCheck>::NAME,
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

pub(crate) use define_baseless_py_dataclass;
pub(crate) use impl_ordered_rich_cmp_for_baseless_py_class;
pub(crate) use impl_unordered_rich_cmp_for_baseless_py_class;
pub(crate) use impl_unordered_rich_cmp_for_derived_py_class;
