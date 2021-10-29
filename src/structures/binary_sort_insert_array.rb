module CoreStructures
  # An array-backed structure that sorts on insertion by using binary search
  # for a given attribute. For example useful for a collection of objects that
  # are sorted by a z-index.
  #
  class BinarySortInsertArray
    # - parameter comparsion: Field used for comparison between elements. E.g.
    #   for objects that need to be sorted by their `#z` method, the value here
    #   should be `:z`.
    #
    def initialize(comparison)
      @comparison = comparison
    end

    def <<(new_element)
      insert_index = store.bsearch_index do |existing_element|
        existing_element.public_send(@comparison) >= new_element.public_send(@comparison)
      end

      if insert_index
        store.insert(insert_index, new_element)
      else
        store << new_element
      end

      new_element
    end

    def delete(element_to_delete)
      deletion_index = store.bsearch_index do |element|
        element.public_send(@comparison) >= element_to_delete.public_send(@comparison)
      end

      loop do
        # we've found it
        break if store[deletion_index] == element_to_delete

        # it's not in here
        if store[deletion_index].nil? || store[deletion_index].public_send(@comparison) != element_to_delete.public_send(@comparison)
          deletion_index = nil
          break
        end

        deletion_index = deletion_index + 1
      end

      return if deletion_index.nil?

      store.delete_at(deletion_index)
    end

    def each(&blk)
      store.each(&blk)
    end

    def inspect
      store.inspect
    end

    def empty?
      store.empty?
    end

    def count
      store.count
    end

    def include?(*args)
      store.include?(*args)
    end

    private

    def store
      @store ||= []
    end
  end
end
