/*
 *	$Id$
 */

MLTERM_ICON ICON "mlterm-icon-win32.ico"

#include  <kiklib/kik_def.h>		/* USE_WIN32API */

#if  defined(USE_WIN32API) || defined(USE_LIBSSH2)

#include  "../x_connect_dialog.h"

ConnectDialog DIALOG 20, 20, 134, 126
	STYLE WS_POPUP | WS_DLGFRAME | DS_CENTER
	{
		LTEXT		"List",		-1,		4,  4,  20,  8
		COMBOBOX			IDD_LIST,	24, 4,  106, 48, CBS_SORT | CBS_DROPDOWNLIST | WS_GROUP | WS_TABSTOP
		GROUPBOX	"Protocol",	-1,		4,  20, 126, 24
		RADIOBUTTON	"&SSH",		IDD_SSH,	8,  30, 40,  12
		RADIOBUTTON	"&TELNET",	IDD_TELNET,	48, 30, 40,  12
		RADIOBUTTON	"&RLOGIN",	IDD_RLOGIN,	88, 30, 40,  12
		LTEXT		"Server",	-1,		4,  46, 30,  8
		EDITTEXT			IDD_SERVER,	34, 46, 96, 10, ES_AUTOHSCROLL
		LTEXT		"Port",		-1,		4,  57, 30,  8
		EDITTEXT			IDD_PORT,	34, 57, 96, 10, ES_AUTOHSCROLL
		LTEXT		"User",		-1,		4,  68, 30,  8
		EDITTEXT			IDD_USER,	34, 68, 96, 10, ES_AUTOHSCROLL
		LTEXT		"Pass",		-1,		4,  79, 30,  8
		EDITTEXT			IDD_PASS,	34, 79, 96, 10, ES_PASSWORD | ES_AUTOHSCROLL
		LTEXT		"Encoding",	-1,		4,  90, 30,  8
		EDITTEXT			IDD_ENCODING,	34, 90, 96, 10, ES_AUTOHSCROLL
		DEFPUSHBUTTON	"OK",		IDOK,		20, 106, 40,  12
		PUSHBUTTON	"Cancel",	IDCANCEL,	80, 106, 40,  12
	}

#endif	/* USE_WIN32API */
