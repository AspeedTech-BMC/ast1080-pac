#[doc = "Register `VIC410` reader"]
pub type R = crate::R<Vic410Spec>;
#[doc = "Register `VIC410` writer"]
pub type W = crate::W<Vic410Spec>;
#[doc = "Field `VICSIRQCSEL34` reader - VIC_SIRQ_CSEL3_4"]
pub type Vicsirqcsel34R = crate::FieldReader<u32>;
#[doc = "Field `VICSIRQCSEL34` writer - VIC_SIRQ_CSEL3_4"]
pub type Vicsirqcsel34W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL3_4"]
    #[inline(always)]
    pub fn vicsirqcsel34(&self) -> Vicsirqcsel34R {
        Vicsirqcsel34R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL3_4"]
    #[inline(always)]
    pub fn vicsirqcsel34(&mut self) -> Vicsirqcsel34W<Vic410Spec> {
        Vicsirqcsel34W::new(self, 0)
    }
}
#[doc = "Int Routing Select3 4\n\nYou can [`read`](crate::Reg::read) this register and get [`vic410::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic410::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic410Spec;
impl crate::RegisterSpec for Vic410Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic410::R`](R) reader structure"]
impl crate::Readable for Vic410Spec {}
#[doc = "`write(|w| ..)` method takes [`vic410::W`](W) writer structure"]
impl crate::Writable for Vic410Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC410 to value 0"]
impl crate::Resettable for Vic410Spec {}
