#[doc = "Register `VIC20C` reader"]
pub type R = crate::R<Vic20cSpec>;
#[doc = "Register `VIC20C` writer"]
pub type W = crate::W<Vic20cSpec>;
#[doc = "Field `VICSIRQCSEL3` reader - VIC_SIRQ_CSEL_3"]
pub type Vicsirqcsel3R = crate::FieldReader<u32>;
#[doc = "Field `VICSIRQCSEL3` writer - VIC_SIRQ_CSEL_3"]
pub type Vicsirqcsel3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL_3"]
    #[inline(always)]
    pub fn vicsirqcsel3(&self) -> Vicsirqcsel3R {
        Vicsirqcsel3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL_3"]
    #[inline(always)]
    pub fn vicsirqcsel3(&mut self) -> Vicsirqcsel3W<Vic20cSpec> {
        Vicsirqcsel3W::new(self, 0)
    }
}
#[doc = "Int Routing Select 3\n\nYou can [`read`](crate::Reg::read) this register and get [`vic20c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic20c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic20cSpec;
impl crate::RegisterSpec for Vic20cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic20c::R`](R) reader structure"]
impl crate::Readable for Vic20cSpec {}
#[doc = "`write(|w| ..)` method takes [`vic20c::W`](W) writer structure"]
impl crate::Writable for Vic20cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC20C to value 0"]
impl crate::Resettable for Vic20cSpec {}
