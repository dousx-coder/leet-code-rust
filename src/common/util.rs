use crate::common::list_node::ListNode;

///
/// 链表转[Vec]
///
pub fn convert_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = vec![];
    let mut curr = head;
    while let Some(node) = curr {
        vec.push(node.val);
        curr = &node.next;
    }
    vec
}

///
/// [Vec]转链表,返回虚拟头节点
///
pub fn convert_linked_list(vec: &Vec<i32>, dummy_head: bool) -> Option<Box<ListNode>> {
    if vec.len() == 0 {
        return None;
    }
    let mut dummy = Box::new(ListNode::new(-1));
    let mut curr = &mut dummy;
    for i in 0..vec.len() {
        let value = vec[i];
        curr.next = Some(Box::new(ListNode::new(value)));
        curr = curr.next.as_mut().unwrap();
    }
    if dummy_head {
        Some(dummy)
    } else {
        dummy.next
    }
}
