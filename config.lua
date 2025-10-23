use_config = true

local function writeb_serial(adr, x) 
	-- print("Being Written to at", adr, "with", x)
	io.write(string.char(x))
end

local function readb_serial(adr) 
	print("Being Read - Nothing Good")
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
		origin = 500,
		type = "func",

		len = 100,
		readb = readb_serial,
		writeb = writeb_serial,
	}
}


print "This is from the configuration file"
