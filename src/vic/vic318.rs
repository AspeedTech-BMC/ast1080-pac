#[doc = "Register `VIC318` reader"]
pub type R = crate::R<Vic318Spec>;
#[doc = "Register `VIC318` writer"]
pub type W = crate::W<Vic318Spec>;
#[doc = "Field `VICSIRQCSEL26` reader - VIC_SIRQ_CSEL2_6"]
pub type Vicsirqcsel26R = crate::FieldReader<u16>;
#[doc = "Field `VICSIRQCSEL26` writer - VIC_SIRQ_CSEL2_6"]
pub type Vicsirqcsel26W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VIC_SIRQ_CSEL2_6"]
    #[inline(always)]
    pub fn vicsirqcsel26(&self) -> Vicsirqcsel26R {
        Vicsirqcsel26R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VIC_SIRQ_CSEL2_6"]
    #[inline(always)]
    pub fn vicsirqcsel26(&mut self) -> Vicsirqcsel26W<Vic318Spec> {
        Vicsirqcsel26W::new(self, 0)
    }
}
#[doc = "Int Routing Select2 6\n\nYou can [`read`](crate::Reg::read) this register and get [`vic318::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic318::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic318Spec;
impl crate::RegisterSpec for Vic318Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic318::R`](R) reader structure"]
impl crate::Readable for Vic318Spec {}
#[doc = "`write(|w| ..)` method takes [`vic318::W`](W) writer structure"]
impl crate::Writable for Vic318Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC318 to value 0"]
impl crate::Resettable for Vic318Spec {}
