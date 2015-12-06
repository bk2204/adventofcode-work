require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd4')

describe AdventOfCode::D4::Miner do
  it 'should compute the hashes correctly' do
    [
      ['abcdef', 609043],
      ['pqrstuv', 1048970]
    ].each do |(key, result)|
      p = AdventOfCode::D4::Miner.new(key, 5)
      count, hash = p.mine
      expect(count).to eq result
      expect(hash).to match(/^00000/)
    end
  end
end
