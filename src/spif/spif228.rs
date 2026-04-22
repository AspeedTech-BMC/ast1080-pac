#[doc = "Register `SPIF228` reader"]
pub type R = crate::R<Spif228Spec>;
#[doc = "Register `SPIF228` writer"]
pub type W = crate::W<Spif228Spec>;
#[doc = "Field `ADDRLBND10` reader - ADDR_LBND10"]
pub type Addrlbnd10R = crate::FieldReader<u16>;
#[doc = "Field `ADDRLBND10` writer - ADDR_LBND10"]
pub type Addrlbnd10W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRUBND10` reader - ADDR_UBND10"]
pub type Addrubnd10R = crate::FieldReader<u16>;
#[doc = "Field `ADDRUBND10` writer - ADDR_UBND10"]
pub type Addrubnd10W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADDR_LBND10"]
    #[inline(always)]
    pub fn addrlbnd10(&self) -> Addrlbnd10R {
        Addrlbnd10R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADDR_UBND10"]
    #[inline(always)]
    pub fn addrubnd10(&self) -> Addrubnd10R {
        Addrubnd10R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDR_LBND10"]
    #[inline(always)]
    pub fn addrlbnd10(&mut self) -> Addrlbnd10W<Spif228Spec> {
        Addrlbnd10W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADDR_UBND10"]
    #[inline(always)]
    pub fn addrubnd10(&mut self) -> Addrubnd10W<Spif228Spec> {
        Addrubnd10W::new(self, 16)
    }
}
#[doc = "SPIF\\_ADDRBND10\n\nYou can [`read`](crate::Reg::read) this register and get [`spif228::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif228::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif228Spec;
impl crate::RegisterSpec for Spif228Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif228::R`](R) reader structure"]
impl crate::Readable for Spif228Spec {}
#[doc = "`write(|w| ..)` method takes [`spif228::W`](W) writer structure"]
impl crate::Writable for Spif228Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF228 to value 0"]
impl crate::Resettable for Spif228Spec {}
