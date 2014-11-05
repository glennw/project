#[cfg(feature = "featureA")]
extern crate "featureA" as feature;

#[cfg(feature = "featureB")]
extern crate "featureB" as feature;

fn main() {
	feature::lib_function();
}
