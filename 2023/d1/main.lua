---------
---@return table<string> lines
---@return string? errmsg
local function read_input()
    local info = debug.getinfo(1, "S")
    local path = info.source:gsub("^@", "")
    local dir = path:match("(.*[/\\])")
    local file, err = io.open(dir .. "/input.txt", "r")
    if not file then
        return {}, err
    end
    local lines = {}
    for line in file:lines() do
        table.insert(lines, line)
    end
    file:close()
    return lines, nil
end

input, err = read_input()
if err then
    error(err)
end

-- Part 1

local function part1()
    local sum = 0
    for _, line in ipairs(input) do
        ---@type string
        local lineStr = line
        local first = lineStr:match("(%d)")
        local last = lineStr:reverse():match("(%d)")
        sum = sum + ((first * 10) + last)
    end

    print(sum)
end


-- Part 2
local function part2()
    local targets = {
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero",
        "%d" -- Match a single digit, not multiple digits
    }

    local function to_digit(str)
        local map = {
            ["one"] = 1,
            ["two"] = 2,
            ["three"] = 3,
            ["four"] = 4,
            ["five"] = 5,
            ["six"] = 6,
            ["seven"] = 7,
            ["eight"] = 8,
            ["nine"] = 9,
            ["zero"] = 0,
        }
        local res = map[str]
        if not res then
            return tonumber(str)
        end
        return res
    end

    local sum = 0
    for i, line in ipairs(input) do
        local lineStr = line
        local first_match_value = nil
        local first_match_index = math.huge
        local last_match_value = nil
        local last_match_index = 0

        for _, target in ipairs(targets) do
            local start_pos = 1
            while true do
                local s, e = lineStr:find(target, start_pos)
                if not s then
                    break
                end

                local match_str = lineStr:sub(s, e)
                local digit_val = to_digit(match_str)

                if s < first_match_index then
                    first_match_index = s
                    first_match_value = digit_val
                end

                if s >= last_match_index then
                    last_match_index = s
                    last_match_value = digit_val
                end

                start_pos = s + 1
            end
        end


        sum = sum + (first_match_value * 10) + last_match_value
        ::continue::
    end

    print(sum)
end
