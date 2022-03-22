import hello_pyo3_build_config


def test_say_hello(capfd):
    hello_pyo3_build_config.say_hello()
    captured = capfd.readouterr()
    assert captured.out == "Hello, World!\n"
