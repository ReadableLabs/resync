Function Definition
https://262.ecma-international.org/12.0/#sec-function-definitions

function identifier (params) {body}

Arrow Function
https://262.ecma-international.org/12.0/#sec-arrow-function-definitions

Params => ConciseBody

ConciseBody
Same line code
{body}

Method Definitions
https://262.ecma-international.org/12.0/#sec-method-definitions

PropertyName (params) {body}

How I will write the parser:

check for:
	=> and then take_while1 space and then either { or statement
	function and then  take_while1 space and then name and then param and then body
	name and then params and then take while whitespace and then {
and step forward byte + take rest if none is found

if ()
params:
take\_until ) and then 
(value)
(value, value2)
	

