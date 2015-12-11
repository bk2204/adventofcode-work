require 'rspec'
load File.join(File.dirname(__FILE__), '..', 'd5')

describe AdventOfCode::D5::Classifier do
  def nice?(s)
    AdventOfCode::D5::Classifier.new.nice?(s)
  end

  def new_nice?(s)
    AdventOfCode::D5::Classifier.new.new_nice?(s)
  end

  it 'should detect nice strings correctly' do
    expect(nice?('ugknbfddgicrmopn')).to eq true
    expect(nice?('aaa')).to eq true
  end

  it 'should reject string with no double letter' do
    expect(nice?('jchzalrnumimnmhp')).to eq false
  end

  it 'should reject strings with forbidden text (xy)' do
    expect(nice?('haegwjzuvuyypxyu')).to eq false
  end

  it 'should reject strings with insufficient vowels' do
    expect(nice?('dvszwmarrgswjxmb')).to eq false
  end

  it 'should detect new nice strings correctly' do
    expect(new_nice?('qjhvhtzxzqqjkmpb')).to eq true
    expect(new_nice?('xxyxx')).to eq true
  end

  it 'should reject strings without a pair separated by a single letter' do
    expect(new_nice?('uurcxstgmygtbstg')).to eq false
  end

  it 'should reject strings without a pair that repeats twice' do
    expect(new_nice?('ieodomkazucvgmuy')).to eq false
  end
end
