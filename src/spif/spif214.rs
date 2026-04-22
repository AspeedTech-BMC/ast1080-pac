#[doc = "Register `SPIF214` reader"]
pub type R = crate::R<Spif214Spec>;
#[doc = "Register `SPIF214` writer"]
pub type W = crate::W<Spif214Spec>;
#[doc = "Field `ADDRLBND05` reader - ADDR_LBND05"]
pub type Addrlbnd05R = crate::FieldReader<u16>;
#[doc = "Field `ADDRLBND05` writer - ADDR_LBND05"]
pub type Addrlbnd05W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRUBND05` reader - ADDR_UBND05"]
pub type Addrubnd05R = crate::FieldReader<u16>;
#[doc = "Field `ADDRUBND05` writer - ADDR_UBND05"]
pub type Addrubnd05W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADDR_LBND05"]
    #[inline(always)]
    pub fn addrlbnd05(&self) -> Addrlbnd05R {
        Addrlbnd05R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADDR_UBND05"]
    #[inline(always)]
    pub fn addrubnd05(&self) -> Addrubnd05R {
        Addrubnd05R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDR_LBND05"]
    #[inline(always)]
    pub fn addrlbnd05(&mut self) -> Addrlbnd05W<Spif214Spec> {
        Addrlbnd05W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADDR_UBND05"]
    #[inline(always)]
    pub fn addrubnd05(&mut self) -> Addrubnd05W<Spif214Spec> {
        Addrubnd05W::new(self, 16)
    }
}
#[doc = "SPIF\\_ADDRBND05\n\nYou can [`read`](crate::Reg::read) this register and get [`spif214::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif214::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif214Spec;
impl crate::RegisterSpec for Spif214Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif214::R`](R) reader structure"]
impl crate::Readable for Spif214Spec {}
#[doc = "`write(|w| ..)` method takes [`spif214::W`](W) writer structure"]
impl crate::Writable for Spif214Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF214 to value 0"]
impl crate::Resettable for Spif214Spec {}
