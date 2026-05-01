#[doc = "Register `VIC414` reader"]
pub type R = crate::R<Vic414Spec>;
#[doc = "Register `VIC414` writer"]
pub type W = crate::W<Vic414Spec>;
#[doc = "Field `VICSIRQCSEL35` reader - VIC_SIRQ_CSEL3_5"]
pub type Vicsirqcsel35R = crate::FieldReader<u32>;
#[doc = "Field `VICSIRQCSEL35` writer - VIC_SIRQ_CSEL3_5"]
pub type Vicsirqcsel35W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL3_5"]
    #[inline(always)]
    pub fn vicsirqcsel35(&self) -> Vicsirqcsel35R {
        Vicsirqcsel35R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL3_5"]
    #[inline(always)]
    pub fn vicsirqcsel35(&mut self) -> Vicsirqcsel35W<Vic414Spec> {
        Vicsirqcsel35W::new(self, 0)
    }
}
#[doc = "Int Routing Select3 5\n\nYou can [`read`](crate::Reg::read) this register and get [`vic414::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic414::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic414Spec;
impl crate::RegisterSpec for Vic414Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic414::R`](R) reader structure"]
impl crate::Readable for Vic414Spec {}
#[doc = "`write(|w| ..)` method takes [`vic414::W`](W) writer structure"]
impl crate::Writable for Vic414Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC414 to value 0"]
impl crate::Resettable for Vic414Spec {}
