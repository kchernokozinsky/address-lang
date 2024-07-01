struct Person {
    std::string* name;
    int* age;
    Person* married_on;

    Person(const std::string& name_val, int age_val)
        : name(new std::string(name_val)), age(new int(age_val)), married_on(nullptr) {}

    ~Person() {
        delete name;
        delete age;
    }
};

void say_hi(Person* person) {
    if (!person || !person->name) {
        std::cout << "Hello, I have no name :(" << std::endl;
    } else {
        std::cout << "Hello, my name is " << *(person->name) << std::endl;
    }
}

void say_age(Person* person) {
    if (!person || !person->age) {
        std::cout << "I don't know how old I am :(" << std::endl;
    } else {
        std::cout << "I'm " << *(person->age) << " years old" << std::endl;
    }
}

void say_who_you_married_on(Person* person) {
    if (!person || !person->married_on || !person->married_on->name) {
        std::cout << "I'm not married yet" << std::endl;
    } else {
        std::cout << "I'm married to " << *(person->married_on->name) << std::endl;
    }
}

void marry(Person* person1, Person* person2) {
    if (person1 && person2) {
        person1->married_on = person2;
        person2->married_on = person1;
    }
}

int main() {
    Person* alice = new Person("Alice", 23);
    Person* bob = new Person("Bob", 25);

    say_hi(alice);
    say_age(alice);

    marry(alice, bob);

    say_who_you_married_on(alice);

    delete alice;
    delete bob;

    return 0;
}