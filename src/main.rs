pub mod node;
pub mod parent_dir;
pub mod splay;
pub mod expose;
pub mod link;
pub mod cut;
pub mod path_query;
pub mod select;
pub mod debug;

pub mod query;

fn main(){
    //query::path_length::path_length_test();
    //query::diameter::diameter_cut_test();
    //query::center::center_test();
    query::median::median_test();
    query::median::median_easy();
    query::median::median_easy2();
    //query::farthest_vertex::farthest_test();
    //query::diameter::diameter_test();
}
