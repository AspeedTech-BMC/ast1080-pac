#[doc = "Register `GPIO868` reader"]
pub type R = crate::R<Gpio868Spec>;
#[doc = "Register `GPIO868` writer"]
pub type W = crate::W<Gpio868Spec>;
#[doc = "Field `GPIO088WrPrivilegeOfMaster` reader - GPIO088 Write Privilege of Master"]
pub type Gpio088wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO088WrPrivilegeOfMaster` writer - GPIO088 Write Privilege of Master"]
pub type Gpio088wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO089WrPrivilegeOfMaster` reader - GPIO089 Write Privilege of Master"]
pub type Gpio089wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO089WrPrivilegeOfMaster` writer - GPIO089 Write Privilege of Master"]
pub type Gpio089wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO090WrPrivilegeOfMaster` reader - GPIO090 Write Privilege of Master"]
pub type Gpio090wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO090WrPrivilegeOfMaster` writer - GPIO090 Write Privilege of Master"]
pub type Gpio090wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO091WrPrivilegeOfMaster` reader - GPIO091 Write Privilege of Master"]
pub type Gpio091wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO091WrPrivilegeOfMaster` writer - GPIO091 Write Privilege of Master"]
pub type Gpio091wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO088 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio088wr_privilege_of_master(&self) -> Gpio088wrPrivilegeOfMasterR {
        Gpio088wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO089 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio089wr_privilege_of_master(&self) -> Gpio089wrPrivilegeOfMasterR {
        Gpio089wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO090 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio090wr_privilege_of_master(&self) -> Gpio090wrPrivilegeOfMasterR {
        Gpio090wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO091 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio091wr_privilege_of_master(&self) -> Gpio091wrPrivilegeOfMasterR {
        Gpio091wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO088 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio088wr_privilege_of_master(&mut self) -> Gpio088wrPrivilegeOfMasterW<Gpio868Spec> {
        Gpio088wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO089 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio089wr_privilege_of_master(&mut self) -> Gpio089wrPrivilegeOfMasterW<Gpio868Spec> {
        Gpio089wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO090 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio090wr_privilege_of_master(&mut self) -> Gpio090wrPrivilegeOfMasterW<Gpio868Spec> {
        Gpio090wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO091 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio091wr_privilege_of_master(&mut self) -> Gpio091wrPrivilegeOfMasterW<Gpio868Spec> {
        Gpio091wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#22\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio868::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio868::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio868Spec;
impl crate::RegisterSpec for Gpio868Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio868::R`](R) reader structure"]
impl crate::Readable for Gpio868Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio868::W`](W) writer structure"]
impl crate::Writable for Gpio868Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO868 to value 0xffff_ffff"]
impl crate::Resettable for Gpio868Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
