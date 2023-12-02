macro_rules! define_derived_exception {
    (
        $name:ident, $py_module_name:literal, $py_name:literal, $base_class_name:ident
    ) => {
        #[pyo3::pyclass(
            module = $py_module_name, name = $py_name, extends=$base_class_name, frozen, get_all
        )]
        pub(super) struct $name {}

        #[pyo3::pymethods]
        impl $name {
            #[new]
            fn new() -> pyo3::PyClassInitializer<Self> {
                $base_class_name::new().add_subclass(Self {})
            }
        }

        impl From<$name> for pyo3::PyErr {
            fn from(_value: $name) -> Self {
                Self::new::<$name, _>(())
            }
        }

        impl super::traits::Repr for $name {
            fn repr(&self, _py: pyo3::Python<'_>) -> pyo3::PyResult<String> {
                Ok(format!("{}()", <Self as pyo3::PyTypeInfo>::NAME))
            }
        }
    };
    (
        $name:ident, $py_module_name:literal, $py_name:literal, $base_class_name:ident,
        $($field_name:ident: $field_type:ty,)+
    ) => {
        #[pyo3::pyclass(
            module = $py_module_name, name = $py_name, extends=$base_class_name, frozen, get_all
        )]
        pub(super) struct $name {
            $($field_name: $field_type,)+
        }

        #[pyo3::pymethods]
        impl $name {
            #[new]
            #[pyo3(signature = ($($field_name,)+ /))]
            fn new(
                $($field_name: $field_type),+
            ) -> pyo3::PyClassInitializer<Self> {
                $base_class_name::new().add_subclass(Self {
                    $($field_name),+
                })
            }
        }

        impl From<$name> for pyo3::PyErr {
            fn from(value: $name) -> Self {
                Self::new::<$name, _>(($(value.$field_name),+))
            }
        }

        impl super::traits::Repr for $name {
            fn repr(&self, py: pyo3::Python<'_>) -> pyo3::PyResult<String> {
                Ok(
                    format!(
                        "{}({})", <Self as pyo3::PyTypeInfo>::NAME,
                        [
                            $(
                                format!("{}={}",
                                stringify!($field_name), self.$field_name.repr(py)?),
                            )+
                        ].join(", ")
                    )
                )
            }
        }
    }
}

pub(super) use define_derived_exception;
