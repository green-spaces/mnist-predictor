mod data_store;
use data_store::loader;

static TRAINING_DATA: &str = "./../uncompressed-data/train-images-idx3-ubyte";
static TRAINING_LABELS: &str = "./../uncompressed-data/train-labels-idx1-ubyte";
static TEST_DATA: &str = "./../uncompressed-data/t10k-images-idx3-ubyte";
static TEST_LABELS: &str = "./../uncompressed-data/t10k-labels-idx1-ubyte";

fn main() {    

    let images = loader::load_images(TRAINING_DATA);
    let labels = loader::load_labels(TRAINING_LABELS);
    let i = 9;
    println!("{}, {}", labels[i], images[i]);
}

