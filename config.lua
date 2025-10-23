use_config = true

local function writeb_serial(adr, x) 
	print("Being Written to at", adr, "with", x)
end

local function readb_serial(adr) 
	print("Being Read")
	print(adr)
	return 0
end

addresses = {
	bootloader = {
		origin = 0,
		type = "file",

		path = "build/program",
	},
	some_memory = {
		origin = 1000,
		type = "ram",

		len = 1000,
	},
	{
		origin = 300,
		type = "func",

		len = 100,
		readb = readb_serial,
		writeb = writeb_serial,
	}
}


print "What on earth is gonig on here"
