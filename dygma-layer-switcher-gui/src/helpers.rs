pub fn remove_opt_index<T>(vec: &mut Vec<T>, index: &mut Option<usize>) {
    if let Some(i) = index {
        vec.remove(*i);
        *index = None;
    }
}
