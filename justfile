default:
    echo 'Hello, world!'

install_and_log:
    R CMD INSTALL . > R_CMD_INSTALL.txt 2>&1

install_r:
    R CMD INSTALL .

document:
    R -e 'devtools::document()'
