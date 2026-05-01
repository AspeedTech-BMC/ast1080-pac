#[doc = "Register `VIC218` reader"]
pub type R = crate::R<Vic218Spec>;
#[doc = "Register `VIC218` writer"]
pub type W = crate::W<Vic218Spec>;
#[doc = "Field `VICSIRQCSEL6` reader - VIC_SIRQ_CSEL_6"]
pub type Vicsirqcsel6R = crate::FieldReader<u16>;
#[doc = "Field `VICSIRQCSEL6` writer - VIC_SIRQ_CSEL_6"]
pub type Vicsirqcsel6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VIC_SIRQ_CSEL_6"]
    #[inline(always)]
    pub fn vicsirqcsel6(&self) -> Vicsirqcsel6R {
        Vicsirqcsel6R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VIC_SIRQ_CSEL_6"]
    #[inline(always)]
    pub fn vicsirqcsel6(&mut self) -> Vicsirqcsel6W<Vic218Spec> {
        Vicsirqcsel6W::new(self, 0)
    }
}
#[doc = "Int Routing Select 6\n\nYou can [`read`](crate::Reg::read) this register and get [`vic218::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic218::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic218Spec;
impl crate::RegisterSpec for Vic218Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic218::R`](R) reader structure"]
impl crate::Readable for Vic218Spec {}
#[doc = "`write(|w| ..)` method takes [`vic218::W`](W) writer structure"]
impl crate::Writable for Vic218Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC218 to value 0"]
impl crate::Resettable for Vic218Spec {}
