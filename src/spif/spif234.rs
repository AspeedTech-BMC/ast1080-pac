#[doc = "Register `SPIF234` reader"]
pub type R = crate::R<Spif234Spec>;
#[doc = "Register `SPIF234` writer"]
pub type W = crate::W<Spif234Spec>;
#[doc = "Field `ADDRLBND13` reader - ADDR_LBND13"]
pub type Addrlbnd13R = crate::FieldReader<u16>;
#[doc = "Field `ADDRLBND13` writer - ADDR_LBND13"]
pub type Addrlbnd13W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRUBND13` reader - ADDR_UBND13"]
pub type Addrubnd13R = crate::FieldReader<u16>;
#[doc = "Field `ADDRUBND13` writer - ADDR_UBND13"]
pub type Addrubnd13W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADDR_LBND13"]
    #[inline(always)]
    pub fn addrlbnd13(&self) -> Addrlbnd13R {
        Addrlbnd13R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADDR_UBND13"]
    #[inline(always)]
    pub fn addrubnd13(&self) -> Addrubnd13R {
        Addrubnd13R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDR_LBND13"]
    #[inline(always)]
    pub fn addrlbnd13(&mut self) -> Addrlbnd13W<Spif234Spec> {
        Addrlbnd13W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADDR_UBND13"]
    #[inline(always)]
    pub fn addrubnd13(&mut self) -> Addrubnd13W<Spif234Spec> {
        Addrubnd13W::new(self, 16)
    }
}
#[doc = "SPIF\\_ADDRBND13\n\nYou can [`read`](crate::Reg::read) this register and get [`spif234::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif234::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif234Spec;
impl crate::RegisterSpec for Spif234Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif234::R`](R) reader structure"]
impl crate::Readable for Spif234Spec {}
#[doc = "`write(|w| ..)` method takes [`spif234::W`](W) writer structure"]
impl crate::Writable for Spif234Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF234 to value 0"]
impl crate::Resettable for Spif234Spec {}
