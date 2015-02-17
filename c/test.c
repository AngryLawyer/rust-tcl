#include <tcl.h>

int main () {
    Tcl_FindExecutable(NULL);
    Tcl_Obj *obj = Tcl_NewStringObj("HULLO", -1);
    Tcl_IncrRefCount(obj);
    Tcl_DecrRefCount(obj);
}
