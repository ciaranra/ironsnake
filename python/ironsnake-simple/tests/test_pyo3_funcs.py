from ironsnake.simple import simple_rs
from ironsnake.simple.person import Person


def test_hello():
    hello = simple_rs.hello()
    assert isinstance(hello, str)


def test_five():
    assert simple_rs.five() == 5


def test_get_tuple():
    assert simple_rs.get_tuple() == ('Hello, World!', 42, 3.141592653589793)


def test_aggregate_creation():
    agg = simple_rs.aggregate_data()
    # agg = simple_rs.PyAggregate("Hello", 42, 3.14)
    assert agg.text == "Hello, Python!"
    assert agg.int == 42
    assert agg.float_number == 3.141592653589793


def test_person_creation():
    name, age = simple_rs.get_person()
    assert name == "Alice"
    assert age == 30

    person = Person()
    assert person.name == name == "Alice"
    assert person.age == age == 30
