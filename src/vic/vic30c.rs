#[doc = "Register `VIC30C` reader"]
pub type R = crate::R<Vic30cSpec>;
#[doc = "Register `VIC30C` writer"]
pub type W = crate::W<Vic30cSpec>;
#[doc = "Field `VICSIRQCSEL23` reader - VIC_SIRQ_CSEL2_3"]
pub type Vicsirqcsel23R = crate::FieldReader<u32>;
#[doc = "Field `VICSIRQCSEL23` writer - VIC_SIRQ_CSEL2_3"]
pub type Vicsirqcsel23W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL2_3"]
    #[inline(always)]
    pub fn vicsirqcsel23(&self) -> Vicsirqcsel23R {
        Vicsirqcsel23R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL2_3"]
    #[inline(always)]
    pub fn vicsirqcsel23(&mut self) -> Vicsirqcsel23W<Vic30cSpec> {
        Vicsirqcsel23W::new(self, 0)
    }
}
#[doc = "Int Routing Select2 3\n\nYou can [`read`](crate::Reg::read) this register and get [`vic30c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic30c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic30cSpec;
impl crate::RegisterSpec for Vic30cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic30c::R`](R) reader structure"]
impl crate::Readable for Vic30cSpec {}
#[doc = "`write(|w| ..)` method takes [`vic30c::W`](W) writer structure"]
impl crate::Writable for Vic30cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC30C to value 0"]
impl crate::Resettable for Vic30cSpec {}
