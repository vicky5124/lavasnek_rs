# Patch the things that depend on os.environ or sys
# This is very similar to the sysconfig patch

# Only patch if get_config_vars was implemented in this module. Python 3.10
# merged the implementations by importing from sysconfig, so we don't need to
# patch twice.
if get_config_vars.__module__ == __name__:
    project_base = '/opt/python/cp38-cp38/bin'

    def get_makefile_filename():
        return '/opt/python/cp38-cp38/lib/python3.8/config-3.8-arm-linux-gnueabihf/Makefile'

    #__real_init_posix = _init_posix
    def _init_posix():
        old = os.environ.get('_PYTHON_SYSCONFIGDATA_NAME')
        os.environ['_PYTHON_SYSCONFIGDATA_NAME'] = '_sysconfigdata__linux_arm-linux-gnueabihf'
        #try:
        #    return __real_init_posix()
        #finally:
        if old is None:
            del os.environ['_PYTHON_SYSCONFIGDATA_NAME']
        else:
            os.environ['_PYTHON_SYSCONFIGDATA_NAME'] = old

    assert _config_vars is None, "distutils.sysconfig was set up prior to patching?"

#vi: ft=python
