#include <Python.h>


int Alice_and_Bob(int alice, int bob)
{

    printf("%d %d\n", alice, bob);
    return 0;

}


static PyObject *alice_bob(PyObject* self, PyObject* args)
{

    PyObject *alice, *bob;

    if (!PyArg_ParseTuple(args, "OO", &alice, &bob)) {
        return NULL;
    }

    int alice_points = 0;
    int bob_points = 0;

    int n = PyObject_Length(alice);
    if (n < 0) {
        return NULL;
    }

    for (int i = 0; i < n; i++) {
        PyObject *item_alice = PyList_GetItem(alice, i);
        int num_alice = PyLong_AsLong(item_alice); // check inputs.

        PyObject *item_bob = PyList_GetItem(bob, i);
        int num_bob = PyLong_AsLong(item_bob);

        if (num_alice > num_bob){
            alice_points++;
        }
        else{
            bob_points++;
        }
    }

    return Py_BuildValue("i", Alice_and_Bob(alice_points, bob_points));
}


static PyMethodDef customMethods[] = {
    {"alice_and_bob", alice_bob, METH_VARARGS, "Make Alice and Bob exercise" },
    {NULL, NULL, 0, NULL}
};

static struct PyModuleDef exercise = {
    PyModuleDef_HEAD_INIT,
    "exercise",
    "Testing module",
    -1,
    customMethods
};

PyMODINIT_FUNC PyInit_exercise(void)
{
    return PyModule_Create(&exercise);
}
