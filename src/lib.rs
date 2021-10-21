pub mod somemod;
mod someother;
mod catpeasant;

// in the sub-dir
mod boof;

fn usesother() {
    let _ = somemod::somefunc();

    let _ = boof::boof::boofer();
}
