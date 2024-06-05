from typing import List, Tuple

from dataclasses import dataclass, field


def is_increasing(a, b):
    return a < b


def is_decreasing(a, b):
    return a > b


def rearrange_for_max_elements(arr: List[List[int]]):
    if len(arr) > 1:
        for i in range(len(arr[1:])):
            if arr[i - 1][-1] < arr[i][0] and len(arr[i - 1]) <= len(arr[i]):
                last_el = arr[i - 1].pop()
                arr[i] = [last_el, *arr[i]]

    return arr


@dataclass
class Tracer:
    rising: List[int] = field(default_factory=list)
    diminishing: List[int] = field(default_factory=list)

    def append(self, element: int) -> Tuple[bool, "Tracer"]:
        """
        1. If diminishing hasn't started and rising's last element < current:
            add to rising.
        2. If diminishing has started and the last element > current:
            add to diminishing.
        3. Else:
            create a new object
        """
        if not self.rising:
            self.rising.append(element)
        elif not self.diminishing and is_increasing(self.rising[-1], element):
            self.rising.append(element)
        elif (
            len(self.rising) > 0
            and len(self.diminishing) == 0
            and not self.rising[-1] == element
        ):
            self.diminishing.append(element)
        elif len(self.diminishing) > 0 and is_decreasing(self.diminishing[-1], element):
            self.diminishing.append(element)
        else:
            return (True, Tracer(rising=[element]))
        return (False, self)

    @property
    def total_len(self):
        return len(self.rising) + len(self.diminishing)

    @property
    def all_peaks(self):
        return self.rising + self.diminishing


def runner(arr):
    results = [Tracer()]
    for i in arr:
        current_tracer = results[-1]
        appended_to_new, job_obj = current_tracer.append(i)
        if appended_to_new:
            results.append(job_obj)

    all_peaks = [t.all_peaks for t in results]
    # all_peaks = rearrange_for_max_elements(all_peaks)
    print(all_peaks)
    all_peak_len = [len(p) for p in all_peaks]
    return max(all_peak_len)


# def longestPeak(array):
#     # Write your code here.
#     max_peak = runner(array)
#     return max_peak


if __name__ == "__main__":
    test_arr = [5, 4, 3, 2, 1, 2, 1]
    max_peak = runner(test_arr)
    print(max_peak)
    max_peak
