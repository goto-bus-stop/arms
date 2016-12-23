local name = arg[1]
if name == 'arms.main' then
  name = 'arms'
end

io.write('package.preload[\'' .. name .. '\'] = function()\n\n')
io.write(io.read('*all'))
io.write('\n\nend\n')
