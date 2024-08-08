# CKB Linked List Tool

A tool to create a linked list between CKB cells, so that to help users to
build a global registry based on the linked list.

## Description

### Architectural Design: How to build a global registry with a linked list on CKB

A brief description is as the following:

- A CKB global registry is a circular[^1] singly linked list, and each node is a
  live cell on [CKB].

- There must exist a method to let all data, which could be registered on a
  global registry instance, to be [strict total ordered].

- Each live cell stores its own data and the next data that in order.

  - If its own data is the largest data in the list, then set the next data
    to be the smallest data, to make the linked list to be circular[^1].

  - To simplify the logic, the current data is not allowed to be the same as
    the next data.
    It means that the list should have 2 items at least.

Then, sets of continuous nodes in any amount, if any two of them have no
intersection, then they can be modified parallelly.

## Usages

- There are 2 functions to check a continuous part of a linked list:

  - `check_linked_list_with_ordered_items`
  - `check_linked_list_with_unordered_items`

  These 2 functions has a same function signature.

  If possible, use `check_linked_list_with_ordered_items` for more efficient
  and less memory cost. If cells for your contract have to be sorted by
  another field, you can use `check_linked_list_with_unordered_items`.

[CKB]: https://github.com/nervosnetwork/ckb
[strict total ordered]: https://en.wikipedia.org/wiki/Total_order#Strict_and_non-strict_total_orders

[^1]: If the linked list is not circular, it will be difficult to tell
whether a set of items are whole items of the list.
