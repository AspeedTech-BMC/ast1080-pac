#[doc = "Register `VIC200` reader"]
pub type R = crate::R<Vic200Spec>;
#[doc = "Register `VIC200` writer"]
pub type W = crate::W<Vic200Spec>;
#[doc = "Field `VICSIRQCSEL0` reader - VIC_SIRQ_CSEL_0"]
pub type Vicsirqcsel0R = crate::FieldReader<u32>;
#[doc = "Field `VICSIRQCSEL0` writer - VIC_SIRQ_CSEL_0"]
pub type Vicsirqcsel0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL_0"]
    #[inline(always)]
    pub fn vicsirqcsel0(&self) -> Vicsirqcsel0R {
        Vicsirqcsel0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL_0"]
    #[inline(always)]
    pub fn vicsirqcsel0(&mut self) -> Vicsirqcsel0W<Vic200Spec> {
        Vicsirqcsel0W::new(self, 0)
    }
}
#[doc = "Int Routing Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`vic200::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic200::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic200Spec;
impl crate::RegisterSpec for Vic200Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic200::R`](R) reader structure"]
impl crate::Readable for Vic200Spec {}
#[doc = "`write(|w| ..)` method takes [`vic200::W`](W) writer structure"]
impl crate::Writable for Vic200Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC200 to value 0"]
impl crate::Resettable for Vic200Spec {}
