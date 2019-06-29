pub mod node;
pub mod parent_dir;
pub mod splay;
pub mod expose;
pub mod link;
pub mod path_query;
pub mod debug;

pub mod query;

fn main(){
    query::path_length::path_length_test();
    query::diameter::diameter_test();
}
