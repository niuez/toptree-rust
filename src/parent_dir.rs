use std::ptr::NonNull;
use crate::node::*;

pub fn parent_dir_comp<T: Clone>(child: CompNode<T>) -> Option<(usize, NonNull<Compress<T>>)> {
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

pub fn parent_dir_comp_guard<T: Clone>(child: CompNode<T>) -> Option<(usize, NonNull<Compress<T>>)> {
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


pub fn parent_dir_comp_rake<T: Clone>(child: CompNode<T>) -> Option<(usize, NonNull<Rake<T>>)> { 
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


pub fn parent_dir_rake<T: Clone>(child: RakeNode<T>) -> Option<(usize, NonNull<Rake<T>>)> {
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
