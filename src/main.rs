pub mod node;
pub mod parent_dir;
pub mod splay;
pub mod expose;
pub mod link;
pub mod path_query;
pub mod debug;

pub mod query;

fn main(){
    query::diameter::diameter_test();
}
