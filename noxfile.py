import nox


@nox.session
def python(session):
    session.install("pytest")
    session.install("maturin")
    session.run_always("maturin", "develop")
    session.run("pytest")
