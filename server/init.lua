box.cfg({ listen = 3301 })

function functions()
    box.schema.func.create('app.start', { language = 'C', if_not_exists = true })
    box.schema.user.grant('guest', 'execute', 'function', 'app.start', { if_not_exists = true })
end

function schema()
    local file = box.schema.space.create('file', { engine = 'vinyl' } )
    file:format{
        { name = 'id', type = 'string' },
        { name = 'data', type = 'varbinary' },
        { name = 'name', type = 'string' },
    }
    file:create_index('primary', { type = 'TREE', parts = { 1, 'string' } })
end

box.once('bootstrap_bench', function() 
    functions()
    schema()
end)
