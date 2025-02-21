use crate::generator::Generator;
use ast_shaper::items::module_item::ModuleItem;
use ast_shaper::test_utils::fixtures::{struct_with_boxed_field, struct_with_complex_field, struct_with_field_attributes, struct_with_optional_and_boxed_field, struct_with_optional_and_optional_field, struct_with_optional_field, struct_with_ref_counter_and_refcell_field, struct_with_required_field, struct_with_vec_of_primitive_field};
use rstest::fixture;
use std::cell::RefCell;
use std::rc::Rc;

#[fixture]
pub fn generator(
    struct_with_required_field: ModuleItem,
    struct_with_optional_field: ModuleItem,
    struct_with_optional_and_optional_field: ModuleItem,
    struct_with_boxed_field: ModuleItem,
    struct_with_ref_counter_and_refcell_field: ModuleItem,
    struct_with_optional_and_boxed_field: ModuleItem,
    struct_with_vec_of_primitive_field: ModuleItem,
    struct_with_complex_field: ModuleItem,
    struct_with_field_attributes: ModuleItem,
) -> Generator {
    let modules = Rc::new(RefCell::new(vec![
        struct_with_required_field,
        struct_with_optional_field,
        struct_with_optional_and_optional_field,
        struct_with_boxed_field,
        struct_with_ref_counter_and_refcell_field,
        struct_with_optional_and_boxed_field,
        struct_with_vec_of_primitive_field,
        struct_with_complex_field,
        struct_with_field_attributes
    ]));
    Generator::new(modules)
}