from . import simple_rs


class Person:
    # data = rust_module.get_person()
    # p = Person(*data)
    def __init__(self):
        # self.name = name
        # self.age = age
        self.name, self.age = simple_rs.get_person()
