default:
    echo 'Hello, world!'

install_and_log:
    R CMD INSTALL --clean --preclean . > R_CMD_INSTALL.txt 2>&1

install_r:
    R CMD INSTALL --clean --preclean .

document:
    R -e 'devtools::document()'

uninstall:
    R CMD REMOVE hellorustc

test:
    R -e 'devtools::load_all();.Call("ultimate_answer");.Call("hellorustc")'

watch:
    watchexec -w src/bindings.rs "R CMD INSTALL --preclean --clean ."