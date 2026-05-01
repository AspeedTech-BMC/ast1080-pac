#[doc = "Register `VIC210` reader"]
pub type R = crate::R<Vic210Spec>;
#[doc = "Register `VIC210` writer"]
pub type W = crate::W<Vic210Spec>;
#[doc = "Field `VICSIRQCSEL4` reader - VIC_SIRQ_CSEL_4"]
pub type Vicsirqcsel4R = crate::FieldReader<u32>;
#[doc = "Field `VICSIRQCSEL4` writer - VIC_SIRQ_CSEL_4"]
pub type Vicsirqcsel4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL_4"]
    #[inline(always)]
    pub fn vicsirqcsel4(&self) -> Vicsirqcsel4R {
        Vicsirqcsel4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL_4"]
    #[inline(always)]
    pub fn vicsirqcsel4(&mut self) -> Vicsirqcsel4W<Vic210Spec> {
        Vicsirqcsel4W::new(self, 0)
    }
}
#[doc = "Int Routing Select 4\n\nYou can [`read`](crate::Reg::read) this register and get [`vic210::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic210::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic210Spec;
impl crate::RegisterSpec for Vic210Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic210::R`](R) reader structure"]
impl crate::Readable for Vic210Spec {}
#[doc = "`write(|w| ..)` method takes [`vic210::W`](W) writer structure"]
impl crate::Writable for Vic210Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC210 to value 0"]
impl crate::Resettable for Vic210Spec {}
