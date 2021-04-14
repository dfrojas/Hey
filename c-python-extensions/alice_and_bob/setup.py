from distutils.core import setup, Extension

setup(name = 'exercise', version = '1.0', ext_modules = [Extension('exercise', ['alice_and_bob.c'])])
