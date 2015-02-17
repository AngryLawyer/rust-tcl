#include <tcl.h>

int main () {
    Tcl_Obj *obj = Tcl_NewObj();
    Tcl_IncrRefCount(obj);
    Tcl_DecrRefCount(obj);
}
