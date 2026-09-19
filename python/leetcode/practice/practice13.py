import heapq
import random
import math
import bisect
from typing import *
from math import inf, factorial, gcd, lcm
from functools import lru_cache, cache
from heapq import heapify, heappush, heappop
from itertools import accumulate, permutations, combinations
from collections import Counter, deque, defaultdict, OrderedDict


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    # 3904. Smallest Stable Index II
    def firstStableIndex(self, nums: list[int], k: int) -> int:
        max_left, min_right = [], []
        big, small, N = -inf, inf, len(nums)
        for i in range(N):
            big = max(big, nums[i])
            small = min(small, nums[(N - i - 1)])
            max_left.append(big)
            min_right.append(small)

        min_right.reverse()
        for i in range(N):
            val = max_left[i] - min_right[i]
            if val <= k:
                return i
        return -1


def test_solution():
    s = Solution()


if __name__ == "__main__":
    test_solution()
