mod module_a;
use crate::module_a::a::a;
use crate::module_a::module_b::b::b;

fn main() {
    a();
    b();
}
