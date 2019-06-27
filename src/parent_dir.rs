use std::ptr::NonNull;
use crate::node::*;

pub fn parent_dir_comp(child: CompNode) -> Option<(usize, NonNull<Compress>)> {
    unsafe {
        match child.parent() {
            Some(ParentNode::Compress(mut p)) => {
                p.as_mut().push();
                if p.as_ref().guard { None }
                else if p.as_ref().child(0) == child { Some((0, p)) }
                else if p.as_ref().child(1) == child { Some((1, p)) }
                else { None }
            }
            _ => None,
        }
    }
}

pub fn parent_dir_comp_guard(child: CompNode) -> Option<(usize, NonNull<Compress>)> {
    unsafe {
        match child.parent() {
            Some(ParentNode::Compress(mut p)) => {
                p.as_mut().push();
                if p.as_ref().child(0) == child { Some((0, p)) }
                else if p.as_ref().child(1) == child { Some((1, p)) }
                else { None }
            }
            _ => None,
        }
    }
}


pub fn parent_dir_comp_rake(child: CompNode) -> Option<(usize, NonNull<Rake>)> { 
    unsafe { 
        match child.parent() {
            Some(ParentNode::Rake(mut p)) => {
                p.as_mut().push();
                if p.as_ref().child(0) == RakeNode::Leaf(child) { Some((0, p)) }
                else if p.as_ref().child(1) == RakeNode::Leaf(child) { Some((1, p)) }
                else { None }
            }
            _ => None,
        }
    }
}


pub fn parent_dir_rake(child: RakeNode) -> Option<(usize, NonNull<Rake>)> {
    unsafe {
        match child.parent() {
            Some(ParentNode::Rake(mut p)) => {
                p.as_mut().push();
                if p.as_ref().child(0) == child { Some((0, p)) }
                else if p.as_ref().child(1) == child { Some((1, p)) }
                else { None }
            }
            _ => None,
        }
    }
}
