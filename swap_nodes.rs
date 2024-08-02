// Definition der Linked List Node-Struktur
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Überprüfe, ob die Liste leer ist oder nur ein Element enthält
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return head;
    }

    // Erstelle Zeiger auf die ersten beiden Knoten
    let mut first = head.unwrap();
    let mut second = first.next.take().unwrap();

    // Tausche die Knoten
    first.next = second.next.take();
    second.next = Some(first);

    // Rekursiver Aufruf für den Rest der Liste
    second.next = swap_pairs(first.next);

    // Gib den neuen Kopf der Liste zurück
    Some(second)
}
