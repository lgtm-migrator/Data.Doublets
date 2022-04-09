﻿namespace Platform::Data::Doublets::Decorators
{
    template <typename ...> class LinksInnerReferenceExistenceValidator;
    template <typename TLink> class LinksInnerReferenceExistenceValidator<TLink> : public DecoratorBase<TFacade, TDecorated>
    {
        public: LinksInnerReferenceExistenceValidator(ILinks<TLink> &storage) : DecoratorBase(storage) { }

        public: TLink Each(Func<IList<TLink>, TLink> handler, const  LinkType& restriction) override
        {
            storage.EnsureInnerReferenceExists(restriction, "restriction");
            return storage.Each(restriction, handler);
        }

        public: TLink Update(const  LinkType& restriction, const LinkType& substitution) override
        {
            storage.EnsureInnerReferenceExists(restriction, "restriction");
            storage.EnsureInnerReferenceExists(substitution, "substitution");
            return storage.Update(restriction, substitution);
        }

        public: void Delete(const  LinkType& restriction) override
        {
            auto link = restriction[_constants.IndexPart];
            storage.EnsureLinkExists(link, "link");
            storage.Delete(link);
        }
    };
}
