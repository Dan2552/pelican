describe CoreStructures::BinarySortInsertArray do
  let(:comparison) { :to_i }
  let(:described_instance) do
    described_class.new(comparison)
  end

  describe "#<<" do
    subject { nil }

    it "inserts in order of the comparison value" do
      described_instance << 1
      described_instance << 8
      described_instance << 2
      described_instance << 5
      described_instance << 3

      expect(described_instance.inspect)
        .to eq("[1, 2, 3, 5, 8]")
    end

    it "changes #count" do
      expect { described_instance << 1 }
        .to change { described_instance.count }
        .from(0)
        .to(1)

      subject
    end
  end

  describe "#delete" do
    let(:element) { 5 }
    subject { described_instance.delete(element) }

    it "removes the element" do
      element_to_delete =
      described_instance << 1
      described_instance << 8
      described_instance << 2
      described_instance << element
      described_instance << 3

      subject

      expect(described_instance.inspect)
        .to eq("[1, 2, 3, 8]")
    end
  end

  describe "#each" do
    let(:iterated) { [] }
    let(:blk) do
      Proc.new do |element|
        iterated << element
      end
    end
    subject { described_instance.each(&blk) }

    it "iterates each element, in insert order" do
      described_instance << 1
      described_instance << 8
      described_instance << 2

      subject

      expect(iterated).to eq([1,2,8])
    end
  end

  describe "#inspect" do
    subject { described_class.inspect }

    it { is_expected.to be_a(String) }
  end

  describe "#count" do
    subject { described_instance.count }

    it { is_expected.to eq(0) }
  end

  describe "#include?" do
    it "behaves like an array" do
      array = []

      array << 1
      array << 2

      described_instance << 1
      described_instance << 2

      expect(array).to include(1)
      expect(array).to include(2)
      expect(array).to_not include(3)

      expect(described_instance).to include(1)
      expect(described_instance).to include(2)
      expect(described_instance).to_not include(3)
    end
  end
end
