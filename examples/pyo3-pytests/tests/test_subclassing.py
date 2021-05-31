import platform

from pyo3_pytests.subclassing import Subclassable, SubclassableWithParameter

PYPY = platform.python_implementation() == "PyPy"


class SomeSubClass(Subclassable):
    def __str__(self):
        return "SomeSubclass"


def test_subclassing():
    if not PYPY:
        a = SomeSubClass()
        assert str(a) == "SomeSubclass"
        assert type(a) is SomeSubClass


class SubclassWithExtraInitArguments(SubclassableWithParameter):
    def __init__(self, bar):
        print("before super init")
        super().__init__(foo=bar * 2)


def test_subclass_with_init():
    s = SubclassWithExtraInitArguments(10)
