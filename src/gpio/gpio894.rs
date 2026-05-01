#[doc = "Register `GPIO894` reader"]
pub type R = crate::R<Gpio894Spec>;
#[doc = "Register `GPIO894` writer"]
pub type W = crate::W<Gpio894Spec>;
#[doc = "Field `GPIO132WrPrivilegeOfMaster` reader - GPIO132 Write Privilege of Master"]
pub type Gpio132wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO132WrPrivilegeOfMaster` writer - GPIO132 Write Privilege of Master"]
pub type Gpio132wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO133WrPrivilegeOfMaster` reader - GPIO133 Write Privilege of Master"]
pub type Gpio133wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO133WrPrivilegeOfMaster` writer - GPIO133 Write Privilege of Master"]
pub type Gpio133wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO134WrPrivilegeOfMaster` reader - GPIO134 Write Privilege of Master"]
pub type Gpio134wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO134WrPrivilegeOfMaster` writer - GPIO134 Write Privilege of Master"]
pub type Gpio134wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO135WrPrivilegeOfMaster` reader - GPIO135 Write Privilege of Master"]
pub type Gpio135wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO135WrPrivilegeOfMaster` writer - GPIO135 Write Privilege of Master"]
pub type Gpio135wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO132 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio132wr_privilege_of_master(&self) -> Gpio132wrPrivilegeOfMasterR {
        Gpio132wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO133 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio133wr_privilege_of_master(&self) -> Gpio133wrPrivilegeOfMasterR {
        Gpio133wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO134 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio134wr_privilege_of_master(&self) -> Gpio134wrPrivilegeOfMasterR {
        Gpio134wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO135 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio135wr_privilege_of_master(&self) -> Gpio135wrPrivilegeOfMasterR {
        Gpio135wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO132 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio132wr_privilege_of_master(&mut self) -> Gpio132wrPrivilegeOfMasterW<Gpio894Spec> {
        Gpio132wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO133 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio133wr_privilege_of_master(&mut self) -> Gpio133wrPrivilegeOfMasterW<Gpio894Spec> {
        Gpio133wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO134 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio134wr_privilege_of_master(&mut self) -> Gpio134wrPrivilegeOfMasterW<Gpio894Spec> {
        Gpio134wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO135 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio135wr_privilege_of_master(&mut self) -> Gpio135wrPrivilegeOfMasterW<Gpio894Spec> {
        Gpio135wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#33\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio894::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio894::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio894Spec;
impl crate::RegisterSpec for Gpio894Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio894::R`](R) reader structure"]
impl crate::Readable for Gpio894Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio894::W`](W) writer structure"]
impl crate::Writable for Gpio894Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO894 to value 0xffff_ffff"]
impl crate::Resettable for Gpio894Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
