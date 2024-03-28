import subprocess
import sys

from setuptools import find_packages, setup
from setuptools_rust import RustExtension

path_relative_to_source_root = (
    subprocess.check_output(['git', 'rev-parse', '--show-prefix'])
    .decode(sys.getdefaultencoding())
    .rstrip('/\n')
)
project_base_url = (
    f'https://github.com/az-lang/az/{path_relative_to_source_root}'
)
setup(
    packages=find_packages(exclude=('tests', 'tests.*')),
    url=project_base_url,
    rust_extensions=[RustExtension('az._az')],
    zip_safe=False,
)
